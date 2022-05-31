
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1017(_: S2, _: S3) {}
        
        fn test1017() { foo1017(S8, S2, S5, S1, S4); }
    