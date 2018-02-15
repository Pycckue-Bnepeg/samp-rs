# SA:MP SDK
Наилучшие биндинги для разработки плагинов SA:MP, которые вы когда либо встречали!

## Features
В наличии куча всяких крутых плюшек, чтобы создание плагина не убило вас.

После одного раза использования Rust SA:MP SDK тебя будет тошнить от любого плагина, что написан не на этом **крутейшем** SDK.

### Удобные и крутейшие макросы
Зачем объявлять эти вонючие глобальные логгеры и прочие вещи руками?
``` Rust
struct Plugin;

impl Plugin {
    fn load() -> bool {
        log!("Plugin is loaded");
        return true;
    }

    fn amx_load(amx: AMX) -> Cell {
        let natives = natives![
            { "MyFunction", my_function }
        ];

        match amx.register(natives) {
            Ok(_) => log!("Natives is successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        AMX_ERR_NONE
    }

    fn my_function(amx: AMX, player_id: i32) -> Cell {
        return -player_id;
    }
}

new_plugin!(Plugin);
```

Хочешь определить нативную функцию и никогда своими руками не делать грязный парсинг аргументов?
``` Rust
// native: FunctionName(int_arg, &float_arg);
define_native!(Plugin, function_name, int_arg: i32, float_ref_arg: ref f32);

// native: WithoutArguments();
define_native(Plugin, function_name);
```

## Документация
Скорее всего она скоро-скоро появится, но пока что здесь абсолютно ничего нет.

## Пример плагина
А [здесь](https://github.com/ZOTTCE/samp-plugin-example) ты можешь взглянуть на лаконичный код простого плагина, где нет грязного и некрасивого C lang.