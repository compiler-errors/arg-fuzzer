
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1101(_: S5, _: S7) {}
        
        fn test1101() { foo1101(S1, S5, S2, S3, S3); }
    