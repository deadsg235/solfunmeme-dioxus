
 #[cfg(test)]
    mod tests {
        use crate::model::{cluster_store::ClusterStore, AdapterCluster};

        //use super::*;

        fn make_cluster(_name: &str, _endpoint: &str) -> AdapterCluster {
            AdapterCluster::new()
            //#FIXME #6 add name and endpoint
            //name.to_string(), endpoint.to_string()
        }

        #[test]
        fn test_new_store_is_empty() {
            let store = ClusterStore::new(vec![]);
            assert_eq!(store.get_clusters().len(), 0);
            assert_eq!(store.active_cluster(), &AdapterCluster::default());
        }

        #[test]
        fn test_add_cluster_success() {
            let mut store = ClusterStore::new(vec![]);
            let cluster = make_cluster("test", "endpoint");
            assert!(store.add_cluster(cluster.clone()).is_ok());
            assert_eq!(store.get_clusters(), &[cluster]);
        }

        #[test]
        fn test_add_cluster_duplicate_name() {
            let mut store = ClusterStore::new(vec![]);
            let cluster1 = make_cluster("test", "endpoint1");
            let cluster2 = make_cluster("test", "endpoint2");
            assert!(store.add_cluster(cluster1).is_ok());
            assert!(store.add_cluster(cluster2).is_err());
        }

        #[test]
        fn test_add_cluster_duplicate_endpoint() {
            let mut store = ClusterStore::new(vec![]);
            let cluster1 = make_cluster("test1", "endpoint");
            let cluster2 = make_cluster("test2", "endpoint");
            assert!(store.add_cluster(cluster1).is_ok());
            assert!(store.add_cluster(cluster2).is_err());
        }

        #[test]
        fn test_set_active_cluster() {
            let mut store = ClusterStore::new(vec![]);
            let cluster = make_cluster("active", "endpoint");
            store.set_active_cluster(cluster.clone());
            assert_eq!(store.active_cluster(), &cluster);
        }

        #[test]
        fn test_get_cluster_found() {
            let cluster = make_cluster("foo", "bar");
            let store = ClusterStore::new(vec![cluster.clone()]);
            assert_eq!(store.get_cluster("foo"), Some(&cluster));
        }

        #[test]
        fn test_get_cluster_not_found() {
            let store = ClusterStore::new(vec![]);
            assert_eq!(store.get_cluster("missing"), None);
        }

        #[test]
        fn test_add_clusters_success() {
            let mut store = ClusterStore::new(vec![]);
            let clusters = vec![
                make_cluster("a", "1"),
                make_cluster("b", "2"),
            ];
            assert!(store.add_clusters(clusters.clone()).is_ok());
            assert_eq!(store.get_clusters(), clusters.as_slice());
        }

        #[test]
        fn test_add_clusters_with_duplicate() {
            let mut store = ClusterStore::new(vec![]);
            let clusters = vec![
                make_cluster("a", "1"),
                make_cluster("a", "2"),
            ];
            assert!(store.add_clusters(clusters).is_err());
            assert_eq!(store.get_clusters().len(), 1);
        }

        #[test]
        fn test_get_clusters_returns_slice() {
            let clusters = vec![
                make_cluster("x", "y"),
                make_cluster("y", "z"),
            ];
            let store = ClusterStore::new(clusters.clone());
            assert_eq!(store.get_clusters(), clusters.as_slice());
        }
    }
