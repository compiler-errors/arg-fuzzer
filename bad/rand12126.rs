
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12126(_: S2, _: S0, _: S5, _: S2) {}
        
        fn test12126() { foo12126(S1, S2, S3, S5, S6, S8); }
    