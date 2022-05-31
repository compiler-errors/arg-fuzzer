
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5907(_: S4, _: S5, _: S6, _: S7) {}
        
        fn test5907() { foo5907(S2, S2, S4, S6, S3); }
    