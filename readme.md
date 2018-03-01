# SA:MP SDK
Наилучшие биндинги для разработки плагинов SA:MP, которые вы когда либо встречали!

## Features
В наличии куча всяких крутых плюшек, чтобы создание плагина не убило вас.

После одного раза использования Rust SA:MP SDK тебя будет тошнить от любого плагина, что написан не на этом **крутейшем** SDK.

### Удобные и крутейшие макросы
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

Хочешь определить нативную функцию и никогда своими руками не делать грязный парсинг аргументов?
``` Rust
// native: FunctionName(int_arg, &float_arg);
define_native!(function_name, int_arg: i32, float_ref_arg: ref f32);

// native: WithoutArguments();
define_native(function_name);
```

## TODO List
* ~~Следует добавить документацию к тому, что уже есть.~~
* Добавить всевозможные нужные `amx_*` для AMX wrapper.
* Добавить еще крутых макросов (например, `let money = call_native!("GetPlayerMoney", player_id);`).
* ~~Не статичная структура для плагина.~~
* ~~Продумать `amx_GetAddr`. Текущая реализация выглядит такой себе, так как приходится использовать `std::mem::forget` на `Box`, возвращаемый из `AMX::get_address`.~~ Реализовано на данный момент как `AMX::get_address_experemental`.

## Документация
Скорее всего она скоро-скоро появится, но пока что здесь абсолютно ничего нет.

## Пример плагина
А [здесь](https://github.com/ZOTTCE/samp-plugin-example) ты можешь взглянуть на лаконичный код простого плагина, где нет грязного и некрасивого C lang.