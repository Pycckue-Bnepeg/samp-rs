# SA:MP SDK
Наилучшие биндинги для разработки плагинов SA:MP, которые вы когда либо встречали!

## Features
В наличии куча всяких крутых плюшек, чтобы создание плагина не убило вас.

После одного раза использования Rust SA:MP SDK тебя будет тошнить от любого плагина, что написан не на этом **крутейшем** SDK.

### Удобные и крутейшие макросы
#### Создание плагина
Зачем объявлять эти вонючие глобальные логгеры и прочие вещи руками?
``` Rust
struct Plugin {
    version: &'static str,
    amx_count: u32,
}

impl Plugin {
    fn load(&self) -> bool {
        log!("Plugin is loaded. Version: {}", self.version);
        return true;
    }

    fn amx_load(&mut self, amx: AMX) -> Cell {
        let natives = natives![
            { "MyFunction", my_function }
        ];

        match amx.register(natives) {
            Ok(_) => log!("Natives are successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        self.amx_count += 1;

        AMX_ERR_NONE
    }

    fn amx_unload(&mut self, _: AMX) -> Cell {
        self.amx_count -= 1;

        AMX_ERR_NONE
    }

    fn my_function(&self, amx: AMX, player_id: i32) -> AmxResult<Cell> {
        Ok(-player_id)
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            version: "0.1",
            amx_count: 0,
        }
    }
}

new_plugin!(Plugin);

// Так же можно запилить ProcessTick, но ты должен, конечно же, объявить Plugin::process_tick.
new_plugin!(Plugin with process_tick)
```
#### Объявление нативных функций.
Хочешь определить нативную функцию и никогда своими руками не делать грязный парсинг аргументов?
``` Rust
// native: FunctionName(int_arg, &float_arg);
define_native!(function_name, int_arg: i32, float_ref_arg: ref f32);

// native: WithoutArguments();
define_native(function_name);
```

#### Вызов нативных функций и паблик функций.
``` Rust
// Уведомление всех подписчиков о смене никнейма пользователя.
fn notify(&self, amx: AMX, player_id: u32, old_name: String, new_name: String) -> AmxResult<Cell> {
    exec_public!(amx, "OnPlayerNameChanged"; player_id, old_name => string, new_name => string) 
}
```

## TODO List
* Сделать новый samp-plugin-example, который будет отражать все плюсы данного SDK.
* Обновить макрос `expand_args!` под новый `AMX::get_address_experemental`.
* Добавить автоматический парсинг для строк и массивов (внутри `define_native!`).

## Документация
Скорее всего она скоро-скоро появится, но пока что здесь абсолютно ничего нет.

Но ее можно посмотреть в исходном коде, либо установив Cargo и выполнив `cargo doc --no-deps`.

## Пример плагина
А [здесь](https://github.com/ZOTTCE/samp-plugin-example) ты можешь взглянуть на лаконичный код простого плагина, где нет грязного и некрасивого C lang.