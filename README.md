Rus Fritz
======
**Rus Fritz** - консольная программа для помощи перевода на русский язык замечательной шахматной программы Deep Fritz 14.

## Motivation

Впрочем может быть использован и для других аналогичных программ, которые хранят файлы переводов аналогично Fritz. Собственно сподвигло меня написание её что файлы перевода довольно большие, некоторые больше 4-х тысяч строк, какие-то данные удаляются, что-то переименовывается или убирается совсем.
В нашем случае, был уже перевод под оригинальную версию Fritz 14, но при обновлении оболочки вся русификация слетела, и все пункты стали выглядеть как NOTEXT.

#Add Info
Собственно что делает программа: 

Загружает в память новый _английский_ файл, и старый _русифицированный_. Его структура (показана часть английского файла!), такова:
```
...
M_HELP,                                           "&Help";
M_CANCEL,                                         "&Cancel";
M_CLOSE,                                          "&Close";
M_OK,                                             "&OK";
M_DONE,                                           "Done";
...
```
1. Каждому первому (левому), полю, например ищется, на основе его, второе поле (правое) из старого русифицированного файла.
2. Если найдено - сохраняем как есть перевод.
3. Если не найдено:
 * Если флаг --ask не задан то оставлем английское значение из второго поля.
 * Если флаг --ask задан то выводим на экран второе поле и ожидаем когда пользователь введет его перевод и нажмет Enter.
 * Если флаг --ask задан но пользователь просто нажал Enter, - оставляем английское значение перевода, как есть.
4. Результат выводим или на stdout или записываем в  файл.

Сама программа написана на языке [Rust](https://www.rust-lang.org/) и собственно это моя первая программа на этом языке, не судите строго ;-) 

## Waring
Важные замечание по сборке и использованию:
1. Необходимо использовать Nightly (1.5) сборку (Дистрибутивы есть для Mac, Linux, Windows). 
2. Файлы должны быть в UTF-8 (Без BOM). Оригинальные файлы Fritz идут в ANSI. 	Для перекодирования удобнее всего использовать программу [Notepad++](https://notepad-plus-plus.org), которая правда только для  Windows: Открываем файл, - Кодировки - Преобразовать в UTF-8 (Без BOM).
3. После получения файла опять используем "Notepad++" для преобразования обратно в ANSI, по аналогии со вторым пунктом.

## Download
* [Version 0.0.7](https://github.com/Ales999/rus-fritz/archive/rusfritz-0.0.7.zip)
* [Version 0.0.6](https://github.com/Ales999/rus-fritz/archive/0.0.6.zip)

## Get and Compile
```
$ git clone https://github.com/Ales999/rus-fritz.git
Cloning into 'rus-fritz'...
remote: Counting objects: 104, done.
remote: Compressing objects: 100% (44/44), done.
remote: Total 104 (delta 42), reused 103 (delta 41), pack-reused 0
Receiving objects: 100% (104/104), 168.00 KiB | 0 bytes/s, done.
Resolving deltas: 100% (42/42), done.
Checking connectivity... done.

$ cd rus-fritz/
$ cargo build --release
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling rustc-serialize v0.3.16
   Compiling libc v0.1.10
   Compiling regex-syntax v0.2.2
   Compiling strsim v0.3.0
   Compiling memchr v0.1.6
   Compiling aho-corasick v0.3.4
   Compiling regex v0.1.41
   Compiling regex_macros v0.1.21
   Compiling docopt v0.6.74
   Compiling docopt_macros v0.6.74
   Compiling Rusic Fritz v0.0.6
$   
```

## Usage
```
$ cd target/release/
$ ./rus_fritz -?
Unknown flag: '-?'

Usage:
	rus_fritz -e <engfile>  -r <rusfile> ( -o <outfile> | --stdout ) [--askme -q]
	rus_fritz (-h | --help)
	rus_fritz --version
Options:
  -h --help		Show this screen.
  --version		Show version.
  --askme		Ask Me for translate
  -q			Quet mode
$
```

## Contributors

### Contributors on GitHub
* [Contributors](https://github.com/Ales999/rus-fritz/graphs/contributors)


## License 
* see [LICENSE](https://github.com/Ales999/rus-fritz/blob/master/LICENSE) file

## Version 
* Version 0.0.7

## How-to use this code
* see [INSTRUCTIONS](https://github.com/Ales999/rus-fritz/blob/master/INSTRUCTIONS.md) file

## Contact
#### Developer/Company
* Homepage:  http://ales999.livejournal.com
* e-mail:  ales999@gmail.com
* Twitter: [@AMekhanoshin](https://twitter.com/AMekhanoshin "AMekhanoshin on twitter")
* other communication/social media
