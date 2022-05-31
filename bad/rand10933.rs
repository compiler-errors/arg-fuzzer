
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10933(_: S4, _: S8) {}
        
        fn test10933() { foo10933(S1, S4, S5); }
    