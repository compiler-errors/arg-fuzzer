
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12391(_: S6, _: S4, _: S7) {}
        
        fn test12391() { foo12391(S3, S4, S5, S6, S8); }
    