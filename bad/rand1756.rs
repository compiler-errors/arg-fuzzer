
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1756(_: S3, _: S2) {}
        
        fn test1756() { foo1756(S5, S1, S7, S5, S6, S3, S5); }
    