
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1997(_: S4, _: S6, _: S8) {}
        
        fn test1997() { foo1997(S2, S5, S8, S8, S2, S4, S5); }
    