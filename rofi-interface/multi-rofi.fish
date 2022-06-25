#!/usr/bin/env fish

set -l action (echo '1. Setup'\n'2. Create'\n'3. Mark'\n'4. Backup' | rofi -theme rounded-nord-dark -width 270 -no-fixed-num-lines -dmenu -p system:)

switch $action
  case '1. Setup'
    set -l qfile (ls | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')
    multi setup $qfile
  case '2. Create'
    set -l qfile (ls | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')
    multi create $qfile
  case '3. Mark'
    set -l afile (ls | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Given Answers:')
    multi mark $afile
  case '4. Backup'
    set -l qfile (ls | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')
    multi backup $qfile
end	   