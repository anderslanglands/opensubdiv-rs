# opensubdiv-rs
A Rust wrapper for Pixar's OpenSubdiv library version 3.3.3: http://graphics.pixar.com/opensubdiv/docs/intro.html

# Building
opensubdiv-rs requires OpenSubdiv version 3.3.3 already built and installed on your system. You must point the build script to it using the `OPENSUBDIV_ROOT` environment variable, as in:
```bash
env OPENSUBDIV_ROOT=/path/to/OpenSubdiv cargo build
```

# Licence
Copyright 2019 Anders Langlands

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.