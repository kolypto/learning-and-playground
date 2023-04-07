#!/usr/bin/env python

import sys
import pathlib

def main(files: list[str]):
        print('# Flutter\n')

        prev_folder = None
        for file_path in files:
            file = pathlib.Path(file_path)
            
            # Folder header
            this_folder = str(file.parent)
            if this_folder != prev_folder:
                print(f'\n\n\n\n# {this_folder}')
                prev_folder = this_folder

            # File contents
            contents = file.read_text()
            # Markdowns: as is
            if file.suffix == '.md':
                print(contents)
            else:
                md_highlight = {
                    '.ts': 'ts',
                    '.js': 'js',
                }[file.suffix]


                # File header
                print(f'\n\n# {file}\n')
                print(f'```{md_highlight}\n{contents}\n```\n')




files = sys.argv[1:]
main(files)
