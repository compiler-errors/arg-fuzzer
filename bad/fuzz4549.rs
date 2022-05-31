
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4549(_: S4, _: S3, _: S5) {}
        
        fn test4549() { foo4549(S8, S6, S4, S7, S8, S6, S6, S1); }
    