$(document).ready(function () {
    $('#signupForm').submit(function (e) {
        var errors = [];
        $('#formErrors').empty();

        $(this).find('input').each(function () {
            var $input = $(this);
            var value = $input.val();
            var required = $input.data('required');
            var minlength = $input.data('minlength');
            var email = $input.data('email');
            var match = $input.data('match');

            if (required && !value) {
                errors.push($input.attr('name') + ' is required.');
            }

            if (value && minlength && value.length < minlength) {
                errors.push($input.attr('name') + ' must be at least ' + minlength + ' characters.');
            }

            if (value && email && !/^\S+@\S+\.\S+$/.test(value)) {
                errors.push($input.attr('name') + ' must be a valid email address.');
            }

            if (value && match && value !== $(match).val()) {
                errors.push('Confirm password must match the password.');
            }
        });

        if (errors.length > 0) {
            e.preventDefault();
            $.each(errors, function (i, error) {
                $('#formErrors').append($('<div>').text(error));
            });
        }
    });
});
