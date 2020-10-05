import './App.css'
import { add, reverse_bytes  } from '../add/src/lib.rs'

document.getElementById("result").innerText = add(5, 4);

let bytes = new Int8Array(5);
for (let i = 0; i < bytes.length; ++i) {
    bytes[i] = i + 1;
}
console.log(bytes);
bytes = reverse_bytes(bytes);
console.log(bytes);
