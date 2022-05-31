
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo726(_: S4, _: S2, _: S8, _: S7) {}
        
        fn test726() { foo726(S1, S1, S3, S4, S6, S1, S7, S7); }
    