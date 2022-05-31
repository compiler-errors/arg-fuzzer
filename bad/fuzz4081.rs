
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4081(_: S2, _: S4) {}
        
        fn test4081() { foo4081(S8, S1, S2, S5, S2); }
    