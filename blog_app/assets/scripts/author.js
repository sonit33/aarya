import Ajv from "https://cdn.skypack.dev/ajv";

$(function () {
	let toggle_form = function (toggle) {
		$("#upload-button").prop("disabled", toggle);
		$("#submit-button").prop("disabled", toggle);
		$("#photo-field").prop("disabled", toggle);
	};
	let toggle_upload = function (toggle) {
		$("#upload-button").prop("disabled", toggle);
	};
	$("#photo-field").on("change", function () {
		var file = this.files[0];
		console.info("File selected:", file.name);
		var maxSize = 2 * 1024 * 1024; // 2 MB
		var errorField = $("#photo-field-error");
		var uploadButton = $("#upload-button");
		if (file) {
			if (file.name.length > 64) {
				console.warn("File name exceeds 64 characters.");
				errorField.text("File name exceeds 64 characters.");
				$(this).val(""); // Clear the input
				toggle_upload(true);
			} else if (file.size > maxSize) {
				console.warn("File size exceeds 2 MB.");
				errorField.text("File size exceeds 2 MB.");
				$(this).val(""); // Clear the input
				toggle_upload(true);
			} else {
				console.info("File is valid.");
				errorField.text("");
				toggle_upload(false);
			}
		}
	});
	$("#upload-button").on("click", function () {
		var file = $("#photo-field")[0].files[0];
		var formData = new FormData();
		formData.append("photo", file);
		// disable the form while the photo is uploading
		toggle_form(true);
		$.ajax({
			url: "/api/photo",
			type: "POST",
			data: formData,
			processData: false,
			contentType: false,
			success: function (data) {
				console.log(data);
				let paths = [];
				data.forEach((photo) => {
					console.log("Upload successful:", photo);
					$("#profile-photo").attr("src", photo.paths["Thumbnail"]);
					$("#photo-url").val(photo.paths["Original"]);
				});
				toggle_form(false);
				toggle_upload(true);
			},
			error: function (err) {
				console.error("Upload failed:", err);
				toggle_form(false);
				toggle_upload(true);
			},
		});
	});
	$("#submit-button").on("click", function (event) {
		var $this = $(this);
		event.preventDefault();

		var payload = {
			first_name: $("#first_name").val(),
			last_name: $("#last_name").val(),
			email: $("#email").val(),
			bio: $("#bio").val(),
			intro: $("#intro").val(),
			photo_url: $("#photo-url").val(),
		};
		console.log(payload);

		var schema = JSON.parse($("#schema").val());
		const ajv = new Ajv({ allErrors: true });
		const validate = ajv.compile(schema);
		const valid = validate(payload);
		$("#errors").empty();
		if (!valid) {
			validate.errors.forEach((error) => {
				const errorMessage = document.createElement("p");
				errorMessage.classList.add("text-sm", "p-1");
				errorMessage.textContent = `${error.instancePath} ${error.message}`;
				$("#errors").append(errorMessage).show();
			});
		} else {
			$.ajax({
				url: $this.data("url"),
				type: "POST",
				contentType: "application/json",
				data: JSON.stringify(payload),
				success: function (data) {
					console.log(data);
					$("#errors").hide();
					window.location.href = "/admin/authors";
				},
				error: function (err) {
					console.error(err.responseText);
					const errorMessage = document.createElement("p");
					errorMessage.classList.add("text-sm", "p-1");
					errorMessage.textContent = err.responseText;
					$("#errors").append(errorMessage).show();
				},
			});
		}
	});
});
