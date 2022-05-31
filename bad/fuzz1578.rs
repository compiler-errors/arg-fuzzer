
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1578(_: S6, _: S3, _: S2) {}
        
        fn test1578() { foo1578(S4, S4, S6, S4, S2); }
    