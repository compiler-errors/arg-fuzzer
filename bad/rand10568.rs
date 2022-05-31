
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10568(_: S2, _: S5, _: S8) {}
        
        fn test10568() { foo10568(S7, S6, S1, S5, S4, S3); }
    