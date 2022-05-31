
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3082(_: S1, _: S4, _: S8) {}
        
        fn test3082() { foo3082(S2, S1, S5, S3); }
    