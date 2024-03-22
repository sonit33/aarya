// import $ from 'jquery';

$(document).ready(function () {
    $('#signupForm').submit(function (e) {
        let errors = [];
        $('#formErrors').empty();

        $(this).find('input').each(function () {
            let $input = $(this);
            let value = $input.val();
            let required = $input.data('required');
            let minlength = $input.data('minlength');
            let maxlength = $input.data('maxlength');
            let email = $input.data('email');
            let match = $input.data('match');

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
