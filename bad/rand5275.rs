
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5275(_: S2, _: S7, _: S8) {}
        
        fn test5275() { foo5275(S6, S4, S7, S5, S1); }
    