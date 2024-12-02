DAY ?= # Variable for the specific day to test (optional)


aoc:
ifeq ($(DAY),)
	@echo "Running all tests..."
	@cargo test -- --nocapture
else
	@echo "Running tests for module day$(DAY)..."
	@cargo test day$(DAY) -- --nocapture
endif

help:
	@echo Usage: make aoc [DAY]
	@echo Example: make aoc DAY=1
	@echo DAY: The day to run tests for (e.g., 1, 2, etc.)
	@echo If DAY is not provided, all tests will be run

