
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11281(_: S1, _: S4, _: S5, _: S6, _: S8) {}
        
        fn test11281() { foo11281(S0, S1, S6, S6, S5, S1); }
    