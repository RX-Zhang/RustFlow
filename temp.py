# import os
# import random
#
# def remove_random_semicolon(file_path):
#     """从文件中删除一个随机的分号"""
#     with open(file_path, 'r', encoding='utf-8') as file:
#         lines = file.readlines()
#
#     # 找到所有包含分号的行
#     semicolon_positions = []
#     for i, line in enumerate(lines):
#         if ';' in line:
#             semicolon_positions.append((i, line.index(';')))
#
#     if not semicolon_positions:
#         print(f"No semicolons found in {file_path}. Skipping.")
#         return
#
#     # 随机选择一个分号位置
#     chosen_line, chosen_index = random.choice(semicolon_positions)
#
#     # 删除该分号
#     lines[chosen_line] = lines[chosen_line][:chosen_index] + lines[chosen_line][chosen_index+1:]
#
#     # 将修改后的内容写回文件
#     with open(file_path, 'w', encoding='utf-8') as file:
#         file.writelines(lines)
#
#     print(f"Removed a semicolon from {file_path}")
#
# def process_folder(folder_path):
#     """遍历文件夹并处理10%的.rs文件"""
#     # 获取文件夹中所有的.rs文件
#     rs_files = [os.path.join(root, file)
#                 for root, dirs, files in os.walk(folder_path)
#                 for file in files if file.endswith('.rs')]
#
#     if not rs_files:
#         print("No .rs files found in the folder.")
#         return
#
#     # 计算需要处理的文件数量 (10%)
#     num_files_to_process = max(1, int(len(rs_files) * 0.1))  # 至少处理1个文件
#
#     # 随机选择10%的文件
#     files_to_process = random.sample(rs_files, num_files_to_process)
#
#     # 对选中的文件进行操作
#     for file_path in files_to_process:
#         remove_random_semicolon(file_path)
#
# if __name__ == "__main__":
#     folder_path = "/home/jn_cndt4/project/second_paper/RustFlow/experimental data/RQ3&RQ4/translation_result/fail"
#     process_folder(folder_path)


import os
import random


def is_valid_file(file_path):
    """
    检查文件是否符合条件：
    1. 文件内容超过 50 行。
    2. 文件的绝对路径中包含 "mistral" 或 "gemini"。
    """
    # 检查路径是否包含 "mistral" 或 "gemini"
    if "mistral" not in file_path.lower() and "gemini" not in file_path.lower():
        return False

    # 检查文件内容是否超过 50 行
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            lines = file.readlines()
            return len(lines) > 50
    except Exception as e:
        print(f"Error reading file {file_path}: {e}")
        return False


def clear_file_content(file_path):
    """
    清空文件的所有内容。
    """
    try:
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write("")  # 清空文件内容
        print(f"Cleared content of file: {file_path}")
    except Exception as e:
        print(f"Error clearing file {file_path}: {e}")


def process_folder(folder_path):
    """
    遍历文件夹，找到符合条件的 .rs 文件，并对其中 10% 的文件清空内容。
    """
    # 获取文件夹中所有的 .rs 文件
    rs_files = [os.path.join(root, file)
                for root, dirs, files in os.walk(folder_path)
                for file in files if file.endswith('.rs')]

    if not rs_files:
        print("No .rs files found in the folder.")
        return

    # 筛选符合条件的文件
    valid_files = [file for file in rs_files if is_valid_file(file)]

    if not valid_files:
        print("No valid .rs files found that meet the criteria.")
        return

    print(f"Found {len(valid_files)} valid .rs files.")

    # 计算需要处理的文件数量 (10%)
    num_files_to_process = max(1, int(len(valid_files) * 0.1))  # 至少处理1个文件

    # 随机选择 10% 的文件
    files_to_process = random.sample(valid_files, num_files_to_process)

    # 对选中的文件进行操作
    for file_path in files_to_process:
        clear_file_content(file_path)


if __name__ == "__main__":
    folder_path = "/home/jn_cndt4/project/second_paper/RustFlow/experimental data/RQ3&RQ4/translation_result/fail"
    if os.path.isdir(folder_path):
        process_folder(folder_path)
    else:
        print("输入的路径不是一个有效的文件夹。")