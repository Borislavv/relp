# Имя вашего исполняемого файла
BINARY_NAME=REPL

# Путь к вашему .plist файлу для launchd
PLIST_PATH=/Library/LaunchDaemons/com.jared1.repl.plist

all: build reload

build:
	cargo build --release
	chmod +x target/release/$(BINARY_NAME)

# Перезагрузка сервиса после сборки
reload: stop start

stop:
	@echo "--- Остановка сервиса... ---"
	if sudo launchctl unload $(PLIST_PATH); then \
		echo "--->>> Сервис успешно остановлен."; \
	else \
		echo "--->>> Ошибка при остановке сервиса."; \
		exit 1; \
	fi

start:
	@echo "--- Запуск сервиса... ---"
	if sudo launchctl load -w $(PLIST_PATH); then \
		echo "--->>> Сервис успешно запущен."; \
	else \
		echo "--->>> Ошибка при запуске сервиса."; \
		exit 1; \
	fi

clean:
	cargo clean