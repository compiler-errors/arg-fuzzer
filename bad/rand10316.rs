
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10316(_: S2, _: S4, _: S6) {}
        
        fn test10316() { foo10316(S2, S5, S3, S1, S4, S1); }
    