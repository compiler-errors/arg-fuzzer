
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1975(_: S3, _: S4, _: S5, _: S6, _: S8) {}
        
        fn test1975() { foo1975(S4, S1, S3, S4, S3, S4, S4); }
    