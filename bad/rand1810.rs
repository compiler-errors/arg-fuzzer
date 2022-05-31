
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1810(_: S5, _: S6, _: S8) {}
        
        fn test1810() { foo1810(S2, S3, S4, S6, S7); }
    