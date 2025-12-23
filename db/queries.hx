// Start writing your queries here.
//
// You can use the schema to help you write your queries.
//
// Queries take the form:
//     QUERY {query name}({input name}: {input type}) =>
//         {variable} <- {traversal}
//         RETURN {variable}
//
// Example:
//     QUERY GetUserFriends(user_id: String) =>
//         friends <- N<User>(user_id)::Out<Knows>
//         RETURN friends
//
//
// For more information on how to write queries,
// see the documentation at https://docs.helix-db.com
// or checkout our GitHub at https://github.com/HelixDB/helix-db


// AddPlayer(name, aliases, isSub) -> N::Player 
QUERY CreatePlayer(name: String, aliases: [String], isSub:Boolean) =>
    player <- AddN<Player>({name: name, aliases: aliases, isSub: isSub})
    RETURN player 

// AddPair (fromPlayer, toPlayer, playedOn, order) -> E::With (x2)
// Asuming that these queries can add the reverse edge too (they can't...)
QUERY CreateWith (from: ID, to: ID, playedOn: Date, order: I8) =>
    pair <- AddE<With>({
        playedOn: playedOn,
        order: order
    })::From(from)::To(to)
    RETURN pair

// AddOpp (fromPlayer, toPlayer, pointsScored, pointsConceded, playedOn, order) -> E::Against (x2)
// Asuming that these queries can add the reverse edge too (they can't...)
QUERY CreateAgainst (from: ID, to: ID, playedOn: Date, order: I8, pointsScored:I8, pointsConceded: I8) =>
    opp <- AddE<Against>({
        playedOn: playedOn,
        order: order,
        pointsScored: pointsScored,
        pointsConceded: pointsConceded,
    })::From(from)::To(to)
    RETURN opp

// Add Game (might be a query but will look something like...)
// 1st game on 22/12/2025 - A,B 21-19 C,D
// If any players don't exist, create them
//      ANode = Add(A, [a], false)
//      BNode = Add(B, [Beee], false)
//      CNode = Add(C, [Sea], false)
//      DNode = Add(D, [], false)
// 
// AddPair(A, B) -> creates A->[E::With]->B AND B->[E::With]->A
// AddPair(C, D) -> creates C->[E::With]->D AND D->[E::With]->C
//
// AddOpp(A, C, 21, 19) creates A->[E::Aginst]->C AND C->[E::With]->A
// AddOpp(A, D, 21, 19) creates A->[E::Aginst]->D AND D->[E::With]->A
// AddOpp(B, C, 19, 21) creates B->[E::Aginst]->C AND C->[E::With]->B
// AddOpp(B, D, 19, 21) creates B->[E::Aginst]->D AND D->[E::With]->B

