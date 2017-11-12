$( document ).ready(function() {
    console.log( "document loaded" );
});

$("#searchPatternField").on('paste input keypress keyup keydown', function() {
    $("#results").empty();
    var x = this.value;
    $.post('/search', x, function(result) { 
        result_json = $.parseJSON(result);
        $("#results").empty();
        if(result_json.length > 1) {
            $.each(result_json, function(index, match) {
                $("#results").append(
                    $(document.createElement('li')).html(match.movie)
                );
            });
        }
    });
});