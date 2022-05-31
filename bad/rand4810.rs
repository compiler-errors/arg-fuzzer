
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4810(_: S4, _: S6) {}
        
        fn test4810() { foo4810(S2, S4, S5, S7, S8); }
    