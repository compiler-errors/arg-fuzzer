
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1836(_: S8, _: S6, _: S1, _: S3, _: S4) {}
        
        fn test1836() { foo1836(S6, S7, S8, S5, S6, S6, S7, S5); }
    