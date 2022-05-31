
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4238(_: S3, _: S4, _: S5, _: S8) {}
        
        fn test4238() { foo4238(S1, S1, S6, S6, S3, S1, S5); }
    