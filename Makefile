# Makefile for running chapter quiz tests individually

# Usage:
#   make run_quiz01   # Runs tests for chapter 1
#   make run_quiz02   # Runs tests for chapter 2
#   ...
#   make run_quiz20   # Runs tests for chapter 20

run_quiz01:
	cargo test --test quiz_chapter01

run_quiz02:
	cargo test --test quiz_chapter02

run_quiz03:
	cargo test --test quiz_chapter03

run_quiz04:
	cargo test --test quiz_chapter04

run_quiz05:
	cargo test --test quiz_chapter05

run_quiz06:
	cargo test --test quiz_chapter06

run_quiz07:
	cargo test --test quiz_chapter07

run_quiz08:
	cargo test --test quiz_chapter08

run_quiz09:
	cargo test --test quiz_chapter09

run_quiz10:
	cargo test --test quiz_chapter10

run_quiz11:
	cargo test --test quiz_chapter11

run_quiz12:
	cargo test --test quiz_chapter12

run_quiz13:
	cargo test --test quiz_chapter13

run_quiz14:
	cargo test --test quiz_chapter14

run_quiz15:
	cargo test --test quiz_chapter15

run_quiz16:
	cargo test --test quiz_chapter16

run_quiz17:
	cargo test --test quiz_chapter17

run_quiz18:
	cargo test --test quiz_chapter18

run_quiz19:
	cargo test --test quiz_chapter19

run_quiz20:
	cargo test --test quiz_chapter20

.PHONY: run_quiz01 run_quiz02 run_quiz03 run_quiz04 run_quiz05 run_quiz06 run_quiz07 run_quiz08 run_quiz09 run_quiz10 run_quiz11 run_quiz12 run_quiz13 run_quiz14 run_quiz15 run_quiz16 run_quiz17 run_quiz18 run_quiz19 run_quiz20
