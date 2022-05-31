
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4307(_: S2, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test4307() { foo4307(S1, S3, S5, S1, S1, S1); }
    