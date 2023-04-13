#!/usr/bin/env fish

function main_menu
  switch $argv	
    case '1. Setup'
      set -l qfile (ls *.json | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')
      multi setup $qfile
      run_gui
    case '2. Create'
      set -l qfile (ls *.json | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')
      multi create $qfile
      run_gui
    case '3. Mark'
      set -l afile (ls *.csv | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Given Answers:')
      multi mark $afile
      run_gui
    case '4. Backup'
      set -l qfile (ls *.json | rofi -theme rounded-nord-dark -width 480 -no-fixed-num-lines -dmenu -p 'Questions Database:')	
      multi backup $qfile
      run_gui
    case '5. Exit'
      echo 'Goodbye!'
  end
end   

function run_gui
  set action (echo '1. Setup'\n'2. Create'\n'3. Mark'\n'4. Backup'\n'5. Exit' | rofi -theme rounded-nord-dark -width 270 -no-fixed-num-lines -dmenu -p system:)
  main_menu $action
end

run_gui

	   