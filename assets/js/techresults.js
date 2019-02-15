'use strict'

function displaySearchResults(){

    let queryTech = "";
    if ('URLSearchParams' in window) {
        const searchParams = new URLSearchParams(window.location.search);
        queryTech = searchParams.get("query");
    } else {
        queryTech = legacyGetUrlParameter("query");
    }

    jQuery(".detectjs").remove();
    jQuery(".techposts")
        .filter( function( index ) {
            return $("var#" + queryTech, this).length === 1;
        })
        .css("display", "");
}

function legacyGetUrlParameter(sParam) {
    var sPageURL = window.location.search.substring(1),
        sURLVariables = sPageURL.split('&'),
        sParameterName,
        i;

    for (i = 0; i < sURLVariables.length; i++) {
        sParameterName = sURLVariables[i].split('=');

        if (sParameterName[0] === sParam) {
            return sParameterName[1] === undefined ? true : decodeURIComponent(sParameterName[1]);
        }
    }
};

jQuery(document).ready( function () {
    displaySearchResults();
  });
