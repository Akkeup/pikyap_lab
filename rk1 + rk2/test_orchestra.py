from main import (
    get_orchestras_starting_with,
    get_max_payment_by_orchestra,
    get_conductor_orchestra_links
)


def test_orchestras_starting_with_A():
    result = get_orchestras_starting_with("А")
    assert len(result) == 1
    assert result[0][0] == "Академический симфонический оркестр"
    assert "Аверьянов" in result[0][1]
    assert "Лосев" in result[0][1]


def test_max_payment_sorted_desc():
    result = get_max_payment_by_orchestra()
    assert result[0][1] == 300000
    assert result[0][0] == "Академический симфонический оркестр"


def test_conductor_orchestra_links():
    result = get_conductor_orchestra_links()
    assert ("Большой симфонический оркестр", "Аверьянов") in result
    assert ("Каменный оркестр", "Филатов") in result