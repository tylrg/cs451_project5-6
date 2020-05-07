import * as wasm from "wasm-ppm";

import { memory } from "wasm-ppm/wasm_ppm_bg";

// tired of seeing this alert
//wasm.greet();

// make a javascript function that sets the content of
// an img element by stuffing it with binary
const setImage = (imageBlob) => {
	// we need to get an arrayBuffer
	// that we can then convert to a Uint8Array
	// which we can then pass straight through to rust
	imageBlob.arrayBuffer().then(
		buff => {
			console.log(buff);

			let byteArray = new Uint8Array(buff);

      let imageLength = byteArray.length;

      // we need to know the length (size?) of the image
      // because it's going to be stored in memory
      // and we need to be able to slice out that chunk
      // of memory

			console.log(byteArray);

			let message_to_encode = document.getElementById("msg-send").value;



			let pointerFromRust = wasm.manipulate_image_in_memory(message_to_encode,
        byteArray);

      let bytesFromRust = new Uint8Array(
        memory.buffer,
        pointerFromRust,
        imageLength);

			// now let's go back and stuff the ppm
			// into the javascript
			let blob = new Blob(
				[bytesFromRust],
				{type: 'image/x-portable-pixmap'});

			// stuff these bytes into the
			// img tag on our page
			const url = window.URL.createObjectURL(blob);

			const img = document.getElementById('img-ppm');
			img.src = url;

      // conceptually, what we are doing is
      // instead of stuffing the blob, which contains our
      // ppm data into an image tag, we are going to
      // create 'temporary' link, that download that data
      // and then we are going to force the browser to
      // click the the link, progmatically, and then it shows
      // up as a download
      const tempLink = document.createElement('a');
      tempLink.style.display = 'none';
      tempLink.href = url;
      tempLink.setAttribute('download', "test-image.ppm");

      if (typeof tempLink.download === 'undefined') {
        tempLink.setAttribute('target', '_blank');
      }

      // add the temporary link to the document itself
      document.body.appendChild(tempLink);
      
      // now "click" it
      tempLink.click();

      // now remove the link from the document
      document.body.removeChild(tempLink);



      // this is some firefox hack
      setTimeout(() => {
        window.URL.revokeObjectURL(url);
      }, 100);

		}
	);
}


const setImageForDecode = (imageBlob) => {
	// we need to get an arrayBuffer
	// that we can then convert to a Uint8Array
	// which we can then pass straight through to rust
	imageBlob.arrayBuffer().then(
		buff => {
			console.log(buff);

			let byteArray = new Uint8Array(buff);

			let imageLength = byteArray.length;

			// we need to know the length (size?) of the image
			// because it's going to be stored in memory
			// and we need to be able to slice out that chunk
			// of memory

			console.log(byteArray);

			//let message_to_encode = document.getElementById("msg-send").value;


			let decoded = wasm.decode_message_from_bytes(byteArray);
			console.log(decoded);
			document.getElementById("output-message").innerText = decoded;
			// let pointerFromRust = wasm.manipulate_image_in_memory(message_to_encode,
			// 	byteArray);

			// let bytesFromRust = new Uint8Array(
			// 	memory.buffer,
			// 	pointerFromRust,
			// 	imageLength);

			// now let's go back and stuff the ppm
			// into the javascript
			// let blob = new Blob(
			// 	[bytesFromRust],
			// 	{ type: 'image/x-portable-pixmap' });

			// stuff these bytes into the
			// img tag on our page
			// const url = window.URL.createObjectURL(blob);

			// const img = document.getElementById('img-ppm');
			// img.src = url;

			// conceptually, what we are doing is
			// instead of stuffing the blob, which contains our
			// ppm data into an image tag, we are going to
			// create 'temporary' link, that download that data
			// and then we are going to force the browser to
			// click the the link, progmatically, and then it shows
			// up as a download
			// const tempLink = document.createElement('a');
			// tempLink.style.display = 'none';
			// tempLink.href = url;
			// tempLink.setAttribute('download', "test-image.ppm");

			// if (typeof tempLink.download === 'undefined') {
			// 	tempLink.setAttribute('target', '_blank');
			// }

			// add the temporary link to the document itself
			//document.body.appendChild(tempLink);

			// now "click" it
			//tempLink.click();

			// now remove the link from the document
			//document.body.removeChild(tempLink);

			// this is some firefox hack
			// setTimeout(() => {
			// 	window.URL.revokeObjectURL(url);
			// }, 100);

		}
	);
}

// grab the file from the browser when the user uploads it
// we want the file as an array of bytes
document.getElementById('file-input').addEventListener(
	'change',
	function() {
		var reader = new FileReader();
		var file = this.files[0];

		// async stuff
		// run this function when the reader has fired
		// the online event
		reader.onload = function() {
			var data = new Blob(
				[reader.result],
				{type: 'image/ppm'}
			);

			this.value = '';

			//console.log(data);

			setImage(data);
		};

		// actually read the file in
		reader.readAsArrayBuffer(file);
	},
	false
);

document.getElementById('decode-input').addEventListener(
	'change',
	function () {
		var reader = new FileReader();
		var file = this.files[0];

		// async stuff
		// run this function when the reader has fired
		// the online event
		reader.onload = function () {
			var data = new Blob(
				[reader.result],
				{ type: 'image/ppm' }
			);

			this.value = '';

			//console.log(data);

			setImageForDecode(data);
		};

		// actually read the file in
		reader.readAsArrayBuffer(file);
	},
	false
);


document.getElementById('btn-update').addEventListener(
	'click',
	function() {
		var txtArea = document.getElementById('text-input');
		var inputText = txtArea.value;
		var outputText = wasm.get_text(inputText);
		var outputPre = document.getElementById('output-pre');
		outputPre.textContent = outputText;
	},
	false
);

document.getElementById('get-text').addEventListener(
	'click',
	function () {
		var text = document.getElementById("msg-send").value;
		//var base = wasm.double(text);
		//wasm.double(text);
		//console.log(base);
	},
	false
);
