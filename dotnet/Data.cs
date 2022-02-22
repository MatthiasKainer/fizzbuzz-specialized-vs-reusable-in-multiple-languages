namespace Data {
    record Rule(Func<int, bool> Matches, string Keyword);
}