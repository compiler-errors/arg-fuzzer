
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10676(_: S6, _: S7) {}
        
        fn test10676() { foo10676(S1, S5, S6, S2, S4); }
    