<!-- rsmlui:block["Rml::FileInterface"] refid="class_rml_1_1_file_interface" -->

The abstract base class for application-specific file I/O.

By default, RmlUi will use a file interface implementing the standard C file functions. If this is not sufficient, or your application wants more control over file I/O, this class should be derived, instanced, and installed through Rml::SetFileInterface() before you initialise RmlUi.

C++ include:

```cpp
#include <RmlUi/Core/FileInterface.h>
```

C++ inheritance: `Rml::NonCopyMoveable`
<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Open"] refid="class_rml_1_1_file_interface_1adc22bc8272a3e6f1fc3aa68f2f92c989" -->

Opens a file. 
#### Parameters
* `path` The path to the file to open. 

#### Returns
A valid file handle, or nullptr on failure

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Close"] refid="class_rml_1_1_file_interface_1a7ba44299702df6efa11ff0e52b792b7e" -->

Closes a previously opened file. 
#### Parameters
* `file` The file handle previously opened through [Open()](#open).

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Read"] refid="class_rml_1_1_file_interface_1a50864b19cfebe5e7657c8d5ddac3945c" -->

Reads data from a previously opened file. 
#### Parameters
* `buffer` The buffer to be read into. 

* `size` The number of bytes to read into the buffer. 

* `file` The handle of the file. 

#### Returns
The total number of bytes read into the buffer.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Seek"] refid="class_rml_1_1_file_interface_1adf997b435720c757ba8e6b564c92e37c" -->

Seeks to a point in a previously opened file. 
#### Parameters
* `file` The handle of the file to seek. 

* `offset` The number of bytes to seek. 

* `origin` One of either SEEK_SET (seek from the beginning of the file), SEEK_END (seek from the end of the file) or SEEK_CUR (seek from the current file position). 

#### Returns
True if the operation completed successfully, false otherwise.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Tell"] refid="class_rml_1_1_file_interface_1a1dab7bc8bef171e46c73ef5db899a396" -->

Returns the current position of the file pointer. 
#### Parameters
* `file` The handle of the file to be queried. 

#### Returns
The number of bytes from the origin of the file.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::Length"] refid="class_rml_1_1_file_interface_1a6ed7411f98b9a521b2e93e79c4f15103" -->

Returns the length of the file. The default implementation uses Seek & Tell. 
#### Parameters
* `file` The handle of the file to be queried. 

#### Returns
The length of the file in bytes.

<!-- /rsmlui:block -->

<!-- rsmlui:block["Rml::FileInterface::LoadFile"] refid="class_rml_1_1_file_interface_1ad9e7ff5213b078ad298db961a7c9c80c" -->

Load and return a file. 
#### Parameters
* `path` The path to the file to load. 

* `out_data` The string contents of the file. 

#### Returns
True on success.

<!-- /rsmlui:block -->

