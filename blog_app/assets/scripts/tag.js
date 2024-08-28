import Ajv from "https://cdn.skypack.dev/ajv";

$(function () {
	$("#submit-button").on("click", function (event) {
		var $this = $(this);
		event.preventDefault();

		var payload = {
			name: $("#name").val(),
			description: $("#description").val(),
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
					window.location = "/admin/tags";
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
