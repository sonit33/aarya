import Ajv from "https://cdn.skypack.dev/ajv";

$(function () {
	let toggle_form = function (toggle) {
		$("#upload-button").prop("disabled", toggle);
		$("#body-upload-button").prop("disabled", toggle);
		$("#more-photos-upload-button").prop("disabled", toggle);
		$("#submit-button").prop("disabled", toggle);
		$("#photo-field").prop("disabled", toggle);
		$("#more-photos-field").prop("disabled", toggle);
	};
	let toggle_upload = function (toggle) {
		$("#upload-button").prop("disabled", toggle);
	};
	let toggle_more_upload = function (toggle) {
		$("#more-photos-upload-button").prop("disabled", toggle);
	};
	let toggle_body_upload = function (toggle) {
		$("#body-upload-button").prop("disabled", toggle);
	};
	let tagId = $("#tag-id");
	let authorId = $("#author-id");
	if (tagId) {
		$("#tags").val(tagId.val());
		console.log("Tag ID:", tagId.val());
	}
	if (authorId) {
		$("#authors").val(authorId.val());
		console.log("Author ID:", authorId.val());
	}
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
					$("#hero-image").attr("src", photo.paths["Hero"]);
					$("#hero-image-url").val(photo.paths["Original"]);
					$("#photo-field-info").text(photo.paths["Hero"]);
				});
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
			error: function (err) {
				console.error("Upload failed:", err);
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
		});
	});
	$("#more-photos-field").on("change", function () {
		var files = this.files;
		var maxSize = 2 * 1024 * 1024; // 2 MB
		var errorField = $("#more-photos-field-error");
		var validFiles = true;

		errorField.text(""); // Clear previous error messages

		for (var i = 0; i < files.length; i++) {
			var file = files[i];
			console.info("File selected:", file.name);

			if (file.name.length > 64) {
				console.warn("File name exceeds 64 characters:", file.name);
				errorField.append(
					"File name exceeds 64 characters: " + file.name + "<br>"
				);
				validFiles = false;
			} else if (file.size > maxSize) {
				console.warn("File size exceeds 2 MB:", file.name);
				errorField.append("File size exceeds 2 MB: " + file.name + "<br>");
				validFiles = false;
			} else {
				console.info("File is valid:", file.name);
			}
		}

		if (validFiles) {
			toggle_more_upload(false);
		} else {
			$(this).val(""); // Clear the input if there are invalid files
			toggle_more_upload(true);
		}
	});
	$("#more-photos-upload-button").on("click", function () {
		var files = $("#more-photos-field")[0].files;
		var formData = new FormData();
		for (var i = 0; i < files.length; i++) {
			formData.append("photos[]", files[i]);
		}
		// disable the form while the photo is uploading
		toggle_form(true);
		$.ajax({
			url: $(this).data("url"),
			type: "POST",
			data: formData,
			processData: false,
			contentType: false,
			success: function (data) {
				console.log(data);
				let paths = [];
				$("#more-photos-field-info").empty();
				data.forEach((photo) => {
					paths.push(photo.paths["Original"]);
					$("#more-photos-field-info").append(
						$("<p>").text(photo.paths["Wide"])
					);
				});
				$("#more-photos-url").val(paths.join(","));
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
			error: function (err) {
				console.error("Upload failed:", err);
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
		});
	});
	$("#body-field").on("change", function () {
		var file = this.files[0];
		console.info("File selected:", file.name);
		var maxSize = 512 * 1024; // 500 KB
		var errorField = $("#body-field-error");
		if (file) {
			if (file.name.length > 64) {
				console.warn("File name exceeds 64 characters.");
				errorField.text("File name exceeds 64 characters.");
				$(this).val(""); // Clear the input
				toggle_body_upload(true);
			} else if (file.size > maxSize) {
				console.warn("File size exceeds 500 KB.");
				errorField.text("File size exceeds 500 KB.");
				$(this).val(""); // Clear the input
				toggle_body_upload(true);
			} else {
				console.info("File is valid.");
				errorField.text("");
				toggle_body_upload(false);
			}
		}
	});
	$("#body-upload-button").on("click", function () {
		var file = $("#body-field")[0].files[0];
		var formData = new FormData();
		formData.append("body", file);
		formData.append("existing_key", $("#body-url").val());
		// disable the form while the photo is uploading
		toggle_form(true);

		console.log(formData.get("existing_key"));
		$.ajax({
			url: "/api/markdown",
			type: "POST",
			data: formData,
			processData: false,
			contentType: false,
			success: function (data) {
				console.log(data);
				let paths = [];
				data.forEach((file) => {
					console.log("Upload successful:", file);
					$("#body-field-info").text(file.paths["markdown"]);
					$("#body-url").val(file.paths["markdown"]);
				});
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
			error: function (err) {
				console.error("Upload failed:", err);
				toggle_form(false);
				toggle_upload(true);
				toggle_more_upload(true);
				toggle_body_upload(true);
			},
		});
	});
	$("#submit-button").on("click", function (event) {
		var $this = $(this);
		event.preventDefault();

		var payload = {
			title: $("#title").val(),
			subtitle: $("#subtitle").val(),
			body: $("#body-url").val(),
			keywords: $("#keywords").val(),
			tldr: $("#tldr").val(),
			hero_image: $("#hero-image-url").val(),
			publish_date: Date.parse($("#publish_date").val()) / 1000,
			modified_date: Date.parse($("#modified_date").val()) / 1000,
			author: $("#authors").val(),
			tag: $("#tags").val(),
			permalink: $("#permalink").val(),
			description: $("#description").val(),
		};
		console.log(payload);

		// return;

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
					window.location.href = "/admin/posts";
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
