
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4755(_: S4, _: S5) {}
        
        fn test4755() { foo4755(S7, S4, S3, S3, S4, S2, S2, S2); }
    