import * as wasm from "wasm-ppm";

import { memory } from "wasm-ppm/wasm_ppm_bg";

//encoding image handling
const setImage = (imageBlob) => {
	imageBlob.arrayBuffer().then(
		buff => {
			let byteArray = new Uint8Array(buff);

			let imageLength = byteArray.length;
			
			let message_to_encode = document.getElementById("msg-send").value;//get the message from the input element

			let pointerFromRust = wasm.manipulate_image_in_memory(message_to_encode,
				byteArray);//call the function to encode the message

			//breaking for error, empty array from rust returns 1
			if(pointerFromRust==1){
				location.reload();
				return;
			}

      let bytesFromRust = new Uint8Array(
        memory.buffer,
        pointerFromRust,
        imageLength);

			let blob = new Blob(
				[bytesFromRust],
				{type: 'image/x-portable-pixmap'});

			//establish and "click" url for dowloading
			const url = window.URL.createObjectURL(blob);
			const img = document.getElementById('img-ppm');
			img.src = url;
      const tempLink = document.createElement('a');
      tempLink.style.display = 'none';
      tempLink.href = url;
      tempLink.setAttribute('download', "image.ppm");

      if (typeof tempLink.download === 'undefined') {
        tempLink.setAttribute('target', '_blank');
      }
      document.body.appendChild(tempLink);
      tempLink.click();
      document.body.removeChild(tempLink);
      setTimeout(() => {window.URL.revokeObjectURL(url);}, 100);

		}
	);
}
//decoding image handling
const setImageForDecode = (imageBlob) => {
	imageBlob.arrayBuffer().then(
		buff => {
			let byteArray = new Uint8Array(buff);
			let imageLength = byteArray.length;
			let decoded = wasm.decode_message_from_bytes(byteArray);//get the decoded message from rust,using data from byteArray
			//if we had an error, refresh the page (alert from rust will be shown)
			if (decoded == "ERROR"){
				location.reload();
				return;
			}
			document.getElementById("output-message").innerText = decoded;//display the output message
			
			//create data for txt file to contain (from decoded string)
			if (!("TextEncoder" in window))
				alert("Sorry, this browser does not support TextEncoder...");
			var enc = new TextEncoder();
			let decodedBytes = enc.encode(decoded);//lol

			let blob = new Blob(
				[decodedBytes],
				{ type: 'text/plain' });

			//establish and "click" url for dowloading
			const url = window.URL.createObjectURL(blob);
			const img = document.getElementById('img-ppm');
			img.src = url;
			const tempLink = document.createElement('a');
			tempLink.style.display = 'none';
			tempLink.href = url;
			tempLink.setAttribute('download', "decoded-message");

			if (typeof tempLink.download === 'undefined') {
				tempLink.setAttribute('target', '_blank');
			}
			document.body.appendChild(tempLink);
			tempLink.click();
			document.body.removeChild(tempLink);
			setTimeout(() => { window.URL.revokeObjectURL(url); }, 100);

		}

	);
}


document.getElementById('file-input').addEventListener(
	'change',
	function() {
		var reader = new FileReader();
		var file = this.files[0];
		reader.onload = function() {
			var data = new Blob(
				[reader.result],
				{type: 'image/ppm'}
			);
			this.value = '';
			setImage(data);
		};
		reader.readAsArrayBuffer(file);
	},
	false
);

document.getElementById('decode-input').addEventListener(
	'change',
	function () {
		var reader = new FileReader();
		var file = this.files[0];
		reader.onload = function () {
			var data = new Blob(
				[reader.result],
				{ type: 'image/ppm' }
			);
			this.value = '';
			setImageForDecode(data);
		};
		reader.readAsArrayBuffer(file);
	},
	false
);