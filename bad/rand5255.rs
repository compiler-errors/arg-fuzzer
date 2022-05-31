
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5255(_: S6, _: S5, _: S8) {}
        
        fn test5255() { foo5255(S3, S3, S7, S2, S1, S5, S1); }
    