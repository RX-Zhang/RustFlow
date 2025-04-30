import logging
from abc import abstractmethod
from dataclasses import dataclass, field
import json
import requests
import time

import re
from typing import Any, List, Dict, Tuple, Union
import settings

from overrides import override
from tenacity import (
    retry,
    wait_random_exponential,
    stop_after_delay,
    retry_if_exception_type,
)

from utils import tag

USER = "USER"
ASSISTANT = "ASSISTANT"

MAX_TOKEN: int = 8192

@dataclass
class Prompt:
    """
    Structured representation of the prompt.
    Args:
        history (List[Tuple[str, str]]): A dialogue consisting of a list of role and content pairs.
        preamble (str): A fixed preamble used for the response.
    """
    # Static shared variable
    context: str = ""
    instruction: str = ""
    constraints: List[str] = field(default_factory=list)
    extra_information: str = ""  # Used to store I/O examples (for code repair tasks）
    few_shots:str = ""  # Few shot prompt (for code translation tasks)

    history: List[Tuple[str, str]] = field(default_factory=list)
    preamble: str = ""

    def __str__(self) -> str:
        if not self.constraints:
            return f"""{self.context}
{self.instruction}
{self.few_shots}
{self.extra_information}
"""
        else:
            constraints_str = ""
            for c_id, constraint in enumerate(self.constraints):
                constraints_str = (constraints_str + "\n\t" + str(c_id + 1) + ". " + constraint)
            return f"""{self.context}
{self.instruction}
{self.few_shots}
{self.extra_information}
Here are some constraints included in the <list> tag, please follow them when translating into the final code:
{tag(constraints_str, "list")}
"""

class QueryError(Exception):
    """
    A wrapper around all sorts of errors thrown by LLMs
    """

    pass

class QueryEngine:
    def __init__(self, global_constraints: List[str], cot_version, num_responses) -> None:
        self.global_constraints = global_constraints
        self.cot_version = cot_version
        self.num_responses = num_responses

    @abstractmethod
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
        multi_answer: bool,
        num_responses: int,
    ) -> Union[List[str],str]: ...

    @retry(
        reraise=True,
        retry=retry_if_exception_type(QueryError),
        wait=wait_random_exponential(multiplier=1, max=60),
        stop=stop_after_delay(300),
    )
    def query(
        self,
        prompt: Prompt,
        model_params: Dict[str, Any] = {"temperature": 0.2},
        multi_answer: bool = False
    ) -> Union[List[str],str]:
        return self.raw_query(prompt, model_params, multi_answer,self.num_responses)

    def stringify_prompt(self, prompt: Prompt) -> str:
        """
        Convert the Prompt object to a string in a specific format
        """
        messages = self.messages(prompt)
        prompt_str = ""
        for message in messages:
            role = message["role"]
            content = message["content"]
            prompt_str += f"{role}:\n{content}\n"

        return prompt_str

    def generate_code(
        self,
        prompt: Prompt,
        model_params: Dict[str, Any] = {"temperature": 0.2},
        fix: bool = True,
        history: bool = False
    ) -> Union[Tuple[List[str], List[str]], Tuple[str, str]]:
        '''
        Generate code using llm.
        Args:
            prompt: Prompt information
            model_params: Model parameters
            fix: Function purpose
            history:
        Returns:
            (responses, res): Generated code block
        '''
        constrained_prompt = Prompt(
            context = prompt.context,
            instruction = prompt.instruction,
            constraints = prompt.constraints if history else self.global_constraints + prompt.constraints + settings.test_constraints_prompt,
            extra_information = prompt.extra_information,
            few_shots = prompt.few_shots,
            preamble = prompt.preamble,
            history = prompt.history,
        )
        if fix:
            response = self.query(constrained_prompt, model_params, multi_answer = False)
            return response, QueryEngine.extract(response, self.cot_version)
        else: # generate code
            responses = self.query(constrained_prompt, model_params, multi_answer = True)
            return responses, [QueryEngine.extract(item, self.cot_version) for item in responses]

    @staticmethod
    def extract(response: str, cot_version: str) -> str:
        if cot_version == "go" or cot_version == "llvm" or cot_version == "ast":
            tagged_block = re.search(r"<code>(?P<code>[\s\S]*)</code>", response)
            if tagged_block: # tagged_block 代码块
                return tagged_block["code"]
            backticked_block = re.findall(r'```rust\n([\s\S]*?)```', response)
            if backticked_block:
                return backticked_block[-1]
            return response
        else:
            tagged_block = re.search(r"<code>(?P<code>[\s\S]*)</code>", response)
            if tagged_block:
                return tagged_block["code"]
            backticked_block = re.search(r"```(rust)?(?P<code>[\s\S]*)```", response)
            if backticked_block:
                return backticked_block["code"]
            return response

    def messages(
        self,
        prompt: Union[str, Prompt],
    ) -> List[Dict[str, str]]:
        '''
        Log messages in a fixed format.
        Args:
            prompt: Prompt (str or Prompt).

        Returns:
            messages: {"role": "user/assistant", "content": content}
        '''
        if isinstance(prompt, str):
            messages = [
                {"role": "user", "content": prompt},
            ]
        else:
            messages = []
            for content in prompt.history:
                role, content = content
                if role == USER:
                    messages.append({"role": "user", "content": content})
                elif role == ASSISTANT:
                    messages.append({"role": "assistant", "content": content})
                else:
                    raise ValueError(f"Unidentified role: {role}")

            messages.append({"role": "user", "content": str(prompt)})

            if prompt.preamble:
                messages.append({"role": "assistant", "content": prompt.preamble.rstrip()})# rstrip: 去除字符串右侧的空白字符
        return messages


class Claude3(QueryEngine):
    def __init__(self, global_constraints: List[str], cot_version, num_responses) -> None:
        super().__init__(global_constraints, cot_version, num_responses)
        self.model = "claude-3-sonnet-20240229"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer sk-"}

    def get_multiple_responses(
        self,
        num_responses: int,
        prompt: Union[str, Prompt],
        init_model_params: Dict[str, Any],
    ) -> list[str]:

        results = []
        try:
            for i in range(num_responses):
                model_temperature = init_model_params["temperature"] + (i * 0.4)
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_temperature,
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                        results.append(result)
                        logging.info(f"A query to Claude3 is made with model temperature as follows: {model_temperature}")
                    except ValueError as e:
                        raise QueryError(e)
                else:
                    result = ""
                    results.append(result)
                    logging.error(f"Request to Claude3 failed with status code: {response.status_code}")
        except Exception as e:
            raise QueryError(e)
        return results

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
        multi_answer: bool,
        num_responses: int,
    ) -> Union[List[str],str]:
        if multi_answer:
            try:
                results = self.get_multiple_responses(num_responses, prompt, model_params)
            except Exception as e:
                raise QueryError(e)
            return results
        else:
            result = ""
            try:
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_params["temperature"],
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)
                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                    except ValueError as e:
                        raise QueryError(e)
            except Exception as e:
                raise QueryError(e)
            logging.info(f"A query to Anthropic Claude3 is made with model paramters as follows: {str(model_params)}")
            return result


class GPT4(QueryEngine):
    def __init__(self, global_constraints: List[str], cot_version, num_responses) -> None:
        super().__init__(global_constraints, cot_version, num_responses)
        self.model = "gpt-4-turbo-preview"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer sk-"}

    def get_multiple_responses(
        self,
        num_responses: int,
        prompt: Union[str, Prompt],
        init_model_params: Dict[str, Any],
    ) -> list[str]:

        results = []
        try:
            for i in range(num_responses):
                model_temperature = init_model_params["temperature"] + (i * 0.4)
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_temperature,
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                        results.append(result)
                        logging.info(f"A query to GPT-4 is made with model temperature as follows: {model_temperature}")
                    except ValueError as e:
                        raise QueryError(e)
                else:
                    result = ""
                    results.append(result)
                    logging.error(f"Request to GPT-4 failed with status code: {response.status_code}")
        except Exception as e:
            raise QueryError(e)
        return results


    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
        multi_answer: bool,
        num_responses: int,
    ) -> Union[List[str],str]:
        if multi_answer:
            try:
                results = self.get_multiple_responses(num_responses, prompt, model_params)
            except Exception as e:
                raise QueryError(e)
            return results
        else:
            result = ""
            try:
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_params["temperature"],
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)
                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                    except ValueError as e:
                        raise QueryError(e)
            except Exception as e:
                raise QueryError(e)
            logging.info(f"A query to GPT4 is made with model paramters as follows: {str(model_params)}")
            return result


class Mistral(QueryEngine):
    def __init__(self, global_constraints: List[str], cot_version, num_responses) -> None:
        super().__init__(global_constraints, cot_version, num_responses)
        self.model = "open-mixtral-8x7b"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer sk-"}

    def get_multiple_responses(
        self,
        num_responses: int,
        prompt: Union[str, Prompt],
        init_model_params: Dict[str, Any],
    ) -> list[str]:
        results = []
        try:
            for i in range(num_responses):
                model_temperature = init_model_params["temperature"] + (i * 0.4)
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_temperature,
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                        results.append(result)
                        logging.info(f"A query to Mistral is made with model temperature as follows: {model_temperature}")
                    except ValueError as e:
                        raise QueryError(e)
                else:
                    result = ""
                    results.append(result)
                    logging.error(f"Request to Mistral failed with status code: {response.status_code}")
        except Exception as e:
            raise QueryError(e)
        return results

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
        multi_answer: bool,
        num_responses: int,
    ) -> Union[List[str],str]:
        if multi_answer:
            try:
                results = self.get_multiple_responses(num_responses, prompt, model_params)
            except Exception as e:
                raise QueryError(e)
            return results
        else:
            result = ""
            try:
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_params["temperature"],
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                    except ValueError as e:
                        raise QueryError(e)
            except Exception as e:
                raise QueryError(e)
            logging.info(f"A query to Mistral is made with model paramters as follows: {str(model_params)}")
            return result


class Gemini(QueryEngine):
    def __init__(self, global_constraints: List[str], cot_version, num_responses) -> None:
        super().__init__(global_constraints, cot_version, num_responses)
        self.model = "gemini-1.5-pro-exp-0827"
        self.url = ''
        self.headers = {"content-type": "application/json",
                        "Authorization": "Bearer sk-"}

    def get_multiple_responses(
            self,
            num_responses: int,
            prompt: Union[str, Prompt],
            init_model_params: Dict[str, Any],
    ) -> list[str]:

        results = []
        try:
            for i in range(num_responses):
                model_temperature = init_model_params["temperature"] + (i * 0.4)
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_temperature,
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)

                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                        results.append(result)
                        logging.info(f"A query to Gemini is made with model temperature as follows: {model_temperature}")
                    except ValueError as e:
                        raise QueryError(e)
                else:
                    result = ""
                    results.append(result)
                    logging.error(f"Request to Gemini failed with status code: {response.status_code}")
        except Exception as e:
            raise QueryError(e)
        return results

    @override
    def raw_query(
        self,
        prompt: Union[str, Prompt],
        model_params: Dict[str, Any],
        multi_answer: bool,
        num_responses: int,
    ) -> Union[List[str],str]:
        if multi_answer:
            try:
                results = self.get_multiple_responses(num_responses, prompt, model_params)
            except Exception as e:
                raise QueryError(e)
            return results
        else:
            result = ""
            try:
                data = {
                    "messages": self.messages(prompt),
                    "model": self.model,
                    "temperature": model_params["temperature"],
                    "top_k": 40,
                    "top_p": 0.9,
                }
                data = json.dumps(data)
                response = requests.post(self.url, data=data, headers=self.headers, timeout=30)
                if response.status_code == 200:
                    settings.llm_call_count += 1
                    try:
                        # Parse JSON response
                        response_json = response.json()
                        result = response_json["choices"][0]["message"]["content"]
                    except ValueError as e:
                        raise QueryError(e)
            except Exception as e:
                raise QueryError(e)
            logging.info(f"A query to Gemini is made with model paramters as follows: {str(model_params)}")
            return result


class QueryEngineFactory:
    @staticmethod
    def create_engine(model: str, global_constraints: List[str] = [],cot_version: str = "", num_responses: int = 1) -> QueryEngine:
        match model:
            case "claude3":
                return Claude3(global_constraints,cot_version, num_responses)
            case "gpt4":
                return GPT4(global_constraints,cot_version, num_responses)
            case "mistral":
                return Mistral(global_constraints,cot_version, num_responses)
            case "gemini":
                return Gemini(global_constraints,cot_version, num_responses)
            case _:
                raise ValueError(f"Unknown model: {model}")
