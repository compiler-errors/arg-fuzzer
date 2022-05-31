
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1964(_: S2, _: S8) {}
        
        fn test1964() { foo1964(S1, S2, S3, S4, S5, S6, S7); }
    