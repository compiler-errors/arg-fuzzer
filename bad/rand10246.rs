
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10246(_: S6, _: S1, _: S2, _: S4, _: S2) {}
        
        fn test10246() { foo10246(S7, S4, S0, S6, S6, S6, S4); }
    