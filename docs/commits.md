# Git Commit Messages Cheat Sheet (Conventional Commits)

Формула заголовка коммита:
`type(scope): <verb in imperative mood> <what was changed> [context/reason]`

---

## 1. Типы изменений (`type`)

* **`feat`** — добавление новой функциональности или методов в API.
* **`fix`** — исправление ошибок, паник, зависаний, сдвигов битов.
* **`refactor`** — улучшение структуры кода без изменения его поведения.
* **`perf`** — оптимизация скорости исполнения или расхода памяти.
* **`test`** — добавление, расширение или исправление юнит/интеграционных тестов.
* **`docs`** — изменения в документации, README, комментариях.
* **`chore`** — рутинные задачи: настройка сборки, Cargo.toml, линтеры, CI/CD.

---

## 2. Глаголы-действия (Imperative Verbs)

Всегда используй начальную форму глагола (как команду):

| Глагол | Значение | Шаблон использования |
| :--- | :--- | :--- |
| **`add`** | добавить с нуля | `add <feature> to <module>` |
| **`implement`** | реализовать алгоритм/логику | `implement <standard/logic> for <struct>` |
| **`fix`** | устранить дефект | `fix <bug> in <component>` |
| **`prevent`** | предотвратить ошибку | `prevent <bad state> when <condition>` |
| **`handle`** | корректно обработать случай | `handle <edge case> in <function>` |
| **`ensure`** | гарантировать инвариант | `ensure <state/invariant> before <action>` |
| **`replace`** | заменить одно решение другим | `replace <old approach> with <new approach>` |
| **`simplify`** | упростить конструкцию | `simplify <logic/control flow>` |
| **`remove` / `drop`** | удалить неиспользуемое | `remove <dead code / unused field>` |
| **`optimize`** | ускорить/уменьшить аллокации | `optimize <algorithm> to avoid allocations` |
| **`allow`** | дать возможность/параметр | `allow <option> via <argument>` |
| **`update`** | актуализировать существующее | `update <dependency / logic> to <version / new spec>` |

---

## 3. Готовые шаблоны под типичные задачи

### Новая функциональность (`feat`)
* `feat(icmp): add update_checksum method to IcmpPacket`
* `feat(checksum): implement RFC 1071 internet checksum algorithm`
* `feat(socket): add raw socket binding for IPv4`
* `feat(packet): add echo_reply constructor to IcmpHeader`
* `feat(cli): support custom packet count and timeout flags`

### Исправления ошибок (`fix`)
* `fix(checksum): fix infinite loop in carry bit folding`
* `fix(parser): prevent index out of bounds on empty byte slice`
* `fix(header): ensure checksum field is zeroed before calculation`
* `fix(endianness): correct byte ordering for 16-bit sequence number`
* `fix(socket): handle permission denied error when opening raw socket`

### Рефакторинг и чистота кода (`refactor`)
* `refactor(checksum): replace try_into().unwrap() with safe slice indexing`
* `refactor(icmp): encapsulate header fields and provide getters`
* `refactor(packet): remove intermediate map adapter before for_each`
* `refactor: eliminate redundant carry and lower variables`
* `refactor: extract packet serialization logic into dedicated method`

### Производительность и оптимизация (`perf`)
* `perf(checksum): avoid heap allocations by operating directly on borrowed slices`
* `perf(packet): replace vector allocation with fixed-size array buffer`
* `perf(loop): use chunks_exact to assist compiler auto-vectorization`

### Тестирование (`test`)
* `test(checksum): add test case for odd-length byte slices`
* `test(checksum): verify calculation against known RFC 1071 test vectors`
* `test(packet): add roundtrip serialization and deserialization test`
* `test(icmp): test packet validation on corrupted checksum`

### Рутина и настройки (`chore` / `docs`)
* `chore(deps): add socket2 dependency with raw socket features`
* `chore: configure release profile with LTO and panic abort`
* `chore: remove temporary debug println calls`
* `docs(readme): add build instructions for musl target`
* `docs(code): document 1s complement checksum verification logic`

---

## 4. Конструктор многострочного коммита (Длинная форма)

Если изменение объёмное, оформляй его с детальным описанием (`body`):

```text
feat(icmp): implement packet construction and checksum verification

- Add `compute_checksum` function supporting odd and even byte slices.
- Implement `IcmpPacket::update_checksum` to recalculate header state.
- Add unit tests verifying calculation against known hex vectors.